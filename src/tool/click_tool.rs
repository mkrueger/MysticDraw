use gtk4::{traits::{BoxExt}, gdk::{Key, ModifierType}};
use crate::editor::Editor;
use super::Tool;

pub struct ClickTool {}

impl Tool for ClickTool
{
    fn get_icon_name(&self) -> &'static str { "edit-select" }

    fn add_tool_page(&self, parent: &mut gtk4::Box)
    {
        parent.append(&gtk4::Label::builder().label("Click").build());
    }

    fn handle_key(&self, editor: &'static mut Editor, key: Key, _key_code: u32, _modifier: ModifierType) -> bool
    {
        println!("handle key {}", key);
        match key {
            Key::Down => {
                editor.set_cursor(editor.cursor.pos.x, editor.cursor.pos.y + 1);
                true
            }
            Key::Up => {
                editor.set_cursor(editor.cursor.pos.x, editor.cursor.pos.y - 1);
                true
            }
            Key::Left => {
                editor.set_cursor(editor.cursor.pos.x - 1, editor.cursor.pos.y);
                true
            }
            Key::Right => {
                editor.set_cursor(editor.cursor.pos.x + 1, editor.cursor.pos.y);
                true
            }
            
            Key::Page_Down => {
                // TODO
                false
            }
            
            Key::Page_Up => {
                // TODO
                false
            }
            
            Key::Home | Key::KP_Home => {
                editor.set_cursor(0, editor.cursor.pos.y);
                true
            }
            
            Key::End | Key::KP_End => {
                editor.set_cursor(editor.buf.width as i32 - 1, editor.cursor.pos.y);
                true
            }

            Key::Return | Key::KP_Enter => {
                editor.set_cursor(0,editor.cursor.pos.y + 1);
                true
            }

            /*
            
            							case SDLK_DELETE:
								for (int i = caret.getLogicalX(); i < getCurrentBuffer()->getWidth(); ++i) {
									getCurrentBuffer()->getCharacter(caret.getLogicalY(), i) = getCurrentBuffer()->getCharacter(caret.getLogicalY(), i + 1);
									getCurrentBuffer()->getAttribute(caret.getLogicalY(), i) = getCurrentBuffer()->getAttribute(caret.getLogicalY(), i + 1);
								}
								getCurrentBuffer()->getCharacter(caret.getLogicalY(), getCurrentBuffer()->getWidth() - 1) = ' ';
								getCurrentBuffer()->getAttribute(caret.getLogicalY(), getCurrentBuffer()->getWidth() - 1) = 7;
								break;
							case SDLK_INSERT:
								caret.insertMode() = !caret.insertMode();
								break;
							case SDLK_BACKSPACE:
								if (caret.getLogicalX()>0){
									if (caret.fontMode() && FontTyped && cpos > 0)  {
										caret.getX() -= CursorPos[cpos] - 1;
										for (a=0;a<=CursorPos[cpos];a++)
										for (b=0;b<=FontLibrary::getInstance().maxY;b++) {
											getCurrentBuffer()->getCharacter(caret.getLogicalY() + b, caret.getLogicalX()+a) = getUndoBuffer()->getCharacter(caret.getLogicalY() + b, caret.getLogicalX()+a);
											getCurrentBuffer()->getAttribute(caret.getLogicalY() + b, caret.getLogicalX()+a) = getUndoBuffer()->getAttribute(caret.getLogicalY() + b, caret.getLogicalX()+a);
										}
										cpos--;
									} else {	
										cpos=0;
										caret.getX()--;
										if (caret.insertMode()) {
											for (int i = caret.getLogicalX(); i < getCurrentBuffer()->getWidth(); ++i) {
												getCurrentBuffer()->getCharacter(caret.getLogicalY(), i) = getCurrentBuffer()->getCharacter(caret.getLogicalY(), i + 1);
												getCurrentBuffer()->getAttribute(caret.getLogicalY(), i) = getCurrentBuffer()->getAttribute(caret.getLogicalY(), i + 1);
											}
											getCurrentBuffer()->getCharacter(caret.getLogicalY(), getCurrentBuffer()->getWidth() - 1) = ' ';
											getCurrentBuffer()->getAttribute(caret.getLogicalY(), getCurrentBuffer()->getWidth() - 1) = 7;
										} else  {
											getCurrentBuffer()->getCharacter(caret.getLogicalY(), getCurrentBuffer()->getWidth() - 1) = ' ';
											getCurrentBuffer()->getAttribute(caret.getLogicalY(), getCurrentBuffer()->getWidth() - 1) = 7;
										} 
									}
								}
								break;

            */

            _ => { false }
                /* 
                    a = event.key.keysym.unicode;
                    if (caret.fontMode() && a > 32 && a < 127) {
                         renderFontCharacter(a);
                    } else  {
                    if (caret.fontMode() && FontTyped) {
                        cpos++;
                        CursorPos[cpos]=2;
                    }
                    if (caret.eliteMode()) {
                        typeCharacter(translate(a)); 
                    } else {
                        typeCharacter(a);
                    }
                */
        }
    }
}
