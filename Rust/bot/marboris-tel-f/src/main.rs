
// 7217319321:AAFJ5em6IT8zahesSFf3BGTqE893f7VzPcE
use frankenstein::Api;
use frankenstein::InlineKeyboardButton;
use frankenstein::InlineKeyboardMarkup;
use frankenstein::ReplyMarkup;
use frankenstein::SendMessageParams;
use frankenstein::TelegramApi;

// replace with your token
static TOKEN: &str = "7217319321:AAFJ5em6IT8zahesSFf3BGTqE893f7VzPcE";
// replace with your chat id
static CHAT_ID: i64 = 5_941_066_026;

fn main() {
    let api = Api::new(TOKEN);

    let mut keyboard: Vec<Vec<InlineKeyboardButton>> = Vec::new();

    for i in 1..5 {
        let mut row: Vec<InlineKeyboardButton> = Vec::new();

        for j in 1..5 {
            let name = format!("{i}{j}");
            let button = InlineKeyboardButton::builder()
                .text(name)
                .url("https://github.com/ayrat555/frankenstein")
                .build();

            row.push(button);
        }

        keyboard.push(row);
    }

    let inline_keyboard = InlineKeyboardMarkup::builder()
        .inline_keyboard(keyboard)
        .build();

    let send_message_params = SendMessageParams::builder()
        .chat_id(CHAT_ID)
        .text("Hi fucker!")
        .reply_markup(ReplyMarkup::InlineKeyboardMarkup(inline_keyboard))
        .build();

    api.send_message(&send_message_params).unwrap();
}
