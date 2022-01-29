import 'dotenv/config'

import { Telegraf } from 'telegraf'
const bot = new Telegraf(process.env.BOT_TOKEN)

bot.start((ctx) => ctx.reply('Welcome! Tristan bot here!'))

bot.help((ctx) => ctx.reply('Well, u gotta do smth'))

bot.command('oldschool', (ctx) => {
  ctx.reply('Hello')
})

bot.on('message', (ctx) => {
  ctx.reply('Unknown command')
})

bot.launch().then(() => console.log('working...'))

// Enable graceful stop
process.once('SIGINT', () => bot.stop('SIGINT'))
process.once('SIGTERM', () => bot.stop('SIGTERM'))
