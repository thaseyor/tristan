import { Context, TelegramError } from 'telegraf'
import { botDescription } from './config'
import { isPrivateChat, isNotAdminError } from './helpers'

export const sendDescription = async (ctx: Context) => {
  if (!isPrivateChat(ctx.chat)) {
    await ctx.deleteMessage()
    return
  }
  await ctx.reply(botDescription)
}

export const deleteSystemMessage = async (ctx: Context) => {
  if (!ctx.message) return
  await ctx.deleteMessage(ctx.message.message_id)
}

export const errorHandler = async (err: unknown, ctx: Context) => {
  if (err instanceof TelegramError && isNotAdminError(err)) {
    const msg = await ctx.reply('promote me to admin pls, i cant do my job')
    setTimeout(() => {
      ctx.deleteMessage(msg.message_id)
    }, 2000)

    return
  }

  throw err
}
