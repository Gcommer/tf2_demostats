mod game;
pub mod summarizer;
mod weapon;

use crate::schema::Schema;
use serde::{Deserialize, Serialize};
use summarizer::DemoSummary;
use tf_demo_parser::{demo::header::Header, Demo, DemoParser};

#[derive(Debug, Serialize, Deserialize)]
pub struct DemoOutput {
    pub filename: Option<String>,

    #[serde(flatten)]
    pub header: Header,

    #[serde(flatten)]
    pub summary: DemoSummary,
}

pub fn parse(buffer: &[u8], schema: &Schema) -> tf_demo_parser::Result<DemoOutput> {
    let demo = Demo::new(buffer);
    let handler = summarizer::MatchAnalyzer::new(schema);
    let stream = demo.get_stream();
    let parser = DemoParser::new_with_analyser(stream, handler);

    let (header, summary) = parser.parse()?;
    Ok(DemoOutput {
        header,
        summary,
        filename: None,
    })
}
