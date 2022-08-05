import { Telegraf } from 'telegraf'

import { TOKEN, commands } from './config'
import { commandsEqual } from './helpers'
import { sendDescription, deleteSystemMessage, errorHandler } from './services'

const bot = new Telegraf(TOKEN)

// update commands list
bot.telegram.getMyCommands().then(async (cmnds) => {
  if (commandsEqual(cmnds, commands)) return
  await bot.telegram.setMyCommands(commands)
})

bot.catch(errorHandler)

bot.start(sendDescription)
bot.help(sendDescription)

bot.on('new_chat_members', deleteSystemMessage)
bot.on('left_chat_member', deleteSystemMessage)

bot.launch().then(() => console.info('working...'))

// Enable graceful stop
process.once('SIGINT', () => bot.stop('SIGINT'))
process.once('SIGTERM', () => bot.stop('SIGTERM'))
