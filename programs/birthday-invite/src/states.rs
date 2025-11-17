use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct BirthdayEvent {
    pub creator: Pubkey, 
    pub event_id: u64,
    pub bump: u8,
    #[max_len(128)]
    pub event_name: String,     // Max 128 bytes
    pub event_date: i64,         
    pub coming_count: u32,
    pub busy_count: u32,
    #[max_len(100)]
    pub rsvps: Vec<RSVP>,  // Track who RSVPed
    #[max_len(500)]      
    pub comments: Vec<Comment>,  // All comments
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub struct RSVP {
    pub invited_person: Pubkey,
    pub is_coming: bool,  // true = Coming, false = Busy
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub struct Comment {
    pub comment_author: Pubkey,
    pub comment_id: u64,
    #[max_len(500)]
    pub content: String,    
}