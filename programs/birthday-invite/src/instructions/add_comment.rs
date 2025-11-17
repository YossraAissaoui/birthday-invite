use anchor_lang::prelude::*;
use crate::states::*;
use crate::errors::ErrorCode;

#[derive(Accounts)]
#[instruction(event_id: u64)]
pub struct AddComment<'info> {
    #[account(mut)]
    pub author: Signer<'info>,

    #[account(
        mut,
        seeds = [b"birthday", birthday_event.creator.as_ref(), event_id.to_le_bytes().as_ref()],
        bump = birthday_event.bump
    )]
    pub birthday_event: Account<'info, BirthdayEvent>,
}

pub fn handler(
    ctx: Context<AddComment>,
    _event_id: u64,
    comment_text: String,
) -> Result<()> {
    // Validate comment text length
    require!(
        !comment_text.is_empty() && comment_text.len() <= 500,
        ErrorCode::InvalidComment
    );

    let birthday_event = &mut ctx.accounts.birthday_event;

    // Check if comments list is at max capacity
    require!(
        birthday_event.comments.len() < 500,
        ErrorCode::TooManyComments
    );

    let comment_id = birthday_event.comments.len() as u64;

    birthday_event.comments.push(Comment {
        comment_author: ctx.accounts.author.key(),
        comment_id,
        content: comment_text,
    });

    Ok(())
}