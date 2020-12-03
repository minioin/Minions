// @Author: BlahGeek
// @Date:   2017-04-18
// @Last Modified by:   BlahGeek
// @Last Modified time: 2018-04-08

mod utils;

mod clipboard;
mod custom_script;
mod file_browser;
mod fontawesome;
mod linux_desktop_entry;
mod reload;
mod search_engine;
mod wolframalpha;
mod youdao;

use crate::mcore::{config::Config, item::Item};

pub fn get_action_items(config: &Config) -> Vec<Item> {
    let mut ret: Vec<Item> = vec![];

    ret.append(&mut search_engine::get(config));
    ret.append(&mut file_browser::get(config));
    ret.append(&mut linux_desktop_entry::get(config));
    ret.append(&mut custom_script::get(config));

    ret.push(clipboard::get(config));
    ret.push(youdao::get(config));
    ret.push(wolframalpha::get(config));
    ret.push(reload::get(config));
    ret.push(fontawesome::get(config));

    ret
}
