pub mod widget_utils {
    pub fn select_next_in_list(current_item: usize, list_length: usize) -> usize {
        if current_item != list_length - 1 {
            current_item + 1
        } else {
            0
        }
    }
    
    pub fn select_prev_in_list(current_item: usize, list_length: usize) -> usize {
        if current_item != 0 {
            current_item - 1
        } else {
            list_length - 1
        }
    }
}