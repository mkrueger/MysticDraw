use std::{rc::Rc};

use druid::{Env,  AppDelegate, DelegateCtx, Target, Command, Handled};

use crate::model::{Buffer};

use super::AppState;


#[derive(Debug, Default)]
pub struct Delegate;

impl AppDelegate<AppState> for Delegate {
    fn command(
        &mut self,
        ctx: &mut DelegateCtx,
        _target: Target,
        cmd: &Command,
        data: &mut AppState,
        _env: &Env,
    ) -> Handled {
        if cmd.is(druid::commands::NEW_FILE) {
            let buffer = Buffer::new();
            let editor = crate::model::Editor::new(0, buffer);
            data.editor.push(Rc::new(editor));
            Handled::Yes
        } else  if let Some(info) = cmd.get(druid::commands::OPEN_FILE) {
            let buffer = Buffer::load_buffer(info.path()).unwrap();
            let editor = crate::model::Editor::new(0, buffer);
            data.editor.push(Rc::new(editor));
            Handled::Yes
        } else /*if cmd.is(druid::commands::SAVE_FILE) {
            Handled::Yes
        } else */if let Some(info) = cmd.get(druid::commands::SAVE_FILE_AS) {
            Handled::Yes
        } else {
            Handled::No
        }
    }
}
