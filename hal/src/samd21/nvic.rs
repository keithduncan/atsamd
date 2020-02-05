// More ugent = pre-empt urgencies below
pub enum Urgency {
    NotUrgent  = 0xC0,
    Urgent     = 0x40,
    VeryUrgent = 0x80,
    MostUrgent = 0x00,
}