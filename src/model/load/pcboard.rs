use super::LoadData;

fn conv_ch(ch: u8) -> u8 {
    if (b'0'..=b'9').contains(&ch) {
        return ch - b'0';
    }
    if (b'a'..=b'f').contains(&ch) {
        return ch - b'a';
    }
    if (b'A'..=b'F').contains(&ch) {
        return ch - b'A';
    }
    0
}

#[allow(non_snake_case)]
pub fn display_PCBoard(data: &mut LoadData, ch: u8) -> u8 {
    if data.pcb_color {
        data.pcb_pos += 1;
        if data.pcb_pos < 3 {
            match data.pcb_pos {
                1 => {
                    data.pcb_value = conv_ch(ch);
                    return 0;
                }
                2 => {
                    data.pcb_value = (data.pcb_value << 4) + conv_ch(ch);
                    data.text_attr = data.pcb_value;
                }
                _ => {}
            }
        }
        data.pcb_color = false;
        data.pcb_code = false;
        return 0;
    }

    if data.pcb_code {
        match ch {
            b'@' => {
                data.pcb_code = false;
            }
            b'X' => {
                data.pcb_color = true;
                data.pcb_pos = 0;
            }
            _ => {}
        }
        return 0;
    }
    match ch {
        b'@' => {
            data.pcb_code = true;
            0
        }
        _ => ch,
    }
}
