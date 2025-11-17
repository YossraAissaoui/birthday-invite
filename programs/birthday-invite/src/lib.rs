use anchor_lang::prelude::*;

mod errors;
mod instructions;
mod states;

pub use instructions::*;
pub use states::*;

declare_id!("BDNqgsdtcdnF2Yuyu5PBvp32sK6q3W73ZPV8LD89NwaY");


#[program]
pub mod birthday_invite {
    use super::*;

    pub fn initialize_bday_event(
        ctx: Context<CreateBirthdayEvent>,
        event_id: u64,
        event_name: String,
        event_date: i64,
    ) -> Result<()> {
        instructions::initialize_bday_event::handler(ctx, event_id, event_name, event_date)
    }

    pub fn confirm_attendance(ctx: Context<ConfirmAttendance>, event_id: u64) -> Result<()> {
        instructions::confirm_attendance::handler(ctx, event_id)
    }

    pub fn decline_attendance(ctx: Context<DeclineAttendance>, event_id: u64) -> Result<()> {
        instructions::decline_attendance::handler(ctx, event_id)
    }

    pub fn add_comment(
        ctx: Context<AddComment>,
        event_id: u64,
        comment_text: String,
    ) -> Result<()> {
        instructions::add_comment::handler(ctx, event_id, comment_text)
    }

    pub fn remove_comment(
        ctx: Context<RemoveComment>,
        event_id: u64,
        comment_id: u64,
    ) -> Result<()> {
        instructions::remove_comment::handler(ctx, event_id, comment_id)
    }
}