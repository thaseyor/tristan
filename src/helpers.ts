import { Context, TelegramError } from 'telegraf'

interface Command {
  command: string
  description: string
}

type Chat = Context['chat']

export const isNotAdminError = (error: TelegramError) => {
  const description = error.response.description
  return (
    description === "Bad Request: message can't be deleted for everyone" ||
    description === "Bad Request: message can't be deleted"
  )
}

export const isPrivateChat = (chat: Chat) => chat?.type === 'private'

export const commandsEqual = (commands1: Command[], commands2: Command[]) =>
  commands1.every((cmnd1, index) => {
    const cmnd2 = commands2[index]

    const isEqualCmnd =
      cmnd1.command.replace('/', '') === cmnd2.command.replace('/', '')
    const isEqualDescription = cmnd1.description === cmnd2.description

    return isEqualCmnd && isEqualDescription
  })
