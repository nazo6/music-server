use async_graphql::*;
use futures_util::Stream;
use tokio::sync::mpsc;

use super::output_objects::ScanStatus;
