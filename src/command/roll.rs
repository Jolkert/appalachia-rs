use std::{collections::HashMap, hash::Hash};

use poise::{
	serenity_prelude::{CreateAllowedMentions, CreateEmbed, CreateEmbedFooter, Mentionable},
	CreateReply,
};
use saikoro::{error::ParsingError, evaluation::DiceEvaluation};

use crate::{Context, Error};

/// Roll some dice!
#[poise::command(slash_command, prefix_command)]
pub async fn roll(
	ctx: Context<'_>,
	#[flag]
	#[description = "When true, only you will see the results"]
	hidden: bool,
	#[description = "The dice expression to be evaluated"]
	#[rest]
	dice: String,
) -> Result<(), Error>
{
	let roll_result = saikoro::evaluate(&dice);
	let mut embed = embed_from_roll(&ctx, &dice, &roll_result);
	if hidden && let poise::Context::Prefix(_) = ctx
	{
		embed = embed.footer(CreateEmbedFooter::new(
			"Note: hidden rolls dont't work with non-slash commands!",
		));
	}

	ctx.send(
		CreateReply::default()
			.embed(embed)
			.reply(true)
			.allowed_mentions(CreateAllowedMentions::new())
			.ephemeral(hidden || roll_result.is_err()),
	)
	.await?;
	Ok(())
}

fn embed_from_roll(
	ctx: &Context<'_>,
	input_string: &str,
	roll: &Result<DiceEvaluation, ParsingError>,
) -> CreateEmbed
{
	match roll
	{
		Ok(roll) => CreateEmbed::new()
			.title("The dice have spoken")
			.description(format!(
				"# **{}**\n{} rolled `{input_string}`",
				roll.value,
				ctx.author().mention(),
			))
			.color(crate::DEFAULT_COLOR)
			.fields(roll.roll_groups.iter().map(|group| {
				(
					format!("{}d{}", group.len(), group.faces),
					format!(
						"[{}]",
						group
							.iter()
							.map(|roll| {
								let wrap = (roll.original_value >= group.faces)
									.then_some("**")
									.unwrap_or_default();
								format!("{wrap}{roll}{wrap}")
							})
							.collect::<Vec<_>>()
							.join(", ")
					),
					true,
				)
			})),
		Err(err) => crate::error_embed(format!(
			"Trying to interpret `{input_string}` failed!\n*{}*",
			err.to_string().replace('*', r"\*")
		)),
	}
}

trait InsertPair<K, V>
{
	fn insert_pair(&mut self, pair: (K, V)) -> Option<V>;
}
impl<K: Eq + Hash, V> InsertPair<K, V> for HashMap<K, V>
{
	fn insert_pair(&mut self, pair: (K, V)) -> Option<V>
	{
		self.insert(pair.0, pair.1)
	}
}
