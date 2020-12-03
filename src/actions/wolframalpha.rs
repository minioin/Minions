extern crate reqwest;
extern crate url;

use self::url::Url;

use crate::mcore::{
    action::{Action, ActionResult, PartialAction},
    config::Config,
    errors::*,
    item::Item,
};
use std::sync::Arc;

use crate::actions::utils::open::OpenAction;

struct WolframAlpha {
    appid: String,
}

const API_URL: &'static str = "http://api.wolframalpha.com/v1/result";
const SEARCH_URL: &'static str = "https://www.wolframalpha.com/input/";

impl Action for WolframAlpha {
    fn runnable_arg(&self) -> bool {
        true
    }

    fn run_arg(&self, text: &str) -> ActionResult {
        let url = Url::parse_with_params(API_URL, &[("appid", self.appid.as_str()), ("i", text)])
            .unwrap();
        let response = reqwest::get(url)
            .map_err(|e| Error::with_chain(e, "Failed to send API request"))?
            .text()
            .map_err(|e| Error::with_chain(e, "Failed to get API reply"))?;
        Ok(vec![Item {
            title: response,
            subtitle: Some(text.into()),
            icon: None,
            badge: None,
            priority: 0,
            data: None,
            search_str: None,
            action: Some(Arc::new(PartialAction::new(
                Arc::new(OpenAction {}),
                Url::parse_with_params(SEARCH_URL, &[("i", text)])
                    .unwrap()
                    .to_string(),
                None,
            ))),
        }])
    }
}

pub fn get(config: &Config) -> Item {
    Item {
        title: "WolframAlpha Short Answer".into(),
        subtitle: None,
        icon: None,
        badge: None,
        priority: 0,
        data: None,
        search_str: None,
        action: Some(Arc::new(WolframAlpha {
            appid: config.get::<String>(&["wolframalpha", "appid"]).unwrap(),
        })),
    }
}
