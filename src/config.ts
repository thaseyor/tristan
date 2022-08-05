import 'dotenv/config'

export const TOKEN = process.env.BOT_TOKEN || ''

if (!TOKEN) {
  console.error('Bot token is not defined')
  process.exit(0)
}

export const commands = [
  // { command: 'start', description: '' },
  // { command: 'help', description: 'Info' },
  // { command: 'stats', description: 'Number of connected chats' },
]

export const botDescription =
  'Tristan here!\nAdd me as an administrator in your group!'
