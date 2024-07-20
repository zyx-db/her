pub fn explanation_prompt(query: String, verbose: bool) -> String {
    match verbose {
        true => {
            format!(
"Help explain the following in simple english.
It is likely a unix / powershell command.
Explain what it does, how to use it, and 1-2 short examples.

Here are some examples:
Question)
ls | grep match

Answer)
The command 'ls | grep match' lists non-hidden files and folders in the current directory, filtered to those that contain the search term 'match'. We can use this command to search for files in the current directory, for example, 'ls | grep example', will find files or folders containing example.

Question)
rm -rf dir

Answer)
The command 'rm -rf dir' will remove the file 'dir', or if 'dir' is a folder, it will recursively delete the contents of the folder. The 'r' flag is for recursively deleting contents, and the 'f' flag is for force, meaning it will not ask for confirmation. We can use this command to delete a folder / file without having to be asked for confirmation. For example 'rm -rf git-repo' will delete 'git-repo', without us having to provide any confirmation.

Question)
{}

Answer)
", query
            )
        }
        false => {
            format!(
                "Help explain the following in simple english.
                It is likely a unix / powershell command.
                Explain what it does.

                Here is the concept / command:
                {}
                ",
                query
            )
        }
    }
}

pub fn suggestion_prompt(prev_commands: String, aliases: String) -> String {
    format!(
"Given the terminal history of the user, along with their current aliases, please provide suggestions for new aliases to help speed up their workflows.

History:
{}

Aliases:
{}
", prev_commands, aliases
    )
}

pub fn summarize_prompt(file_contents: String) -> String {
    format!(
"Given the contents of a file please provide a summary. The user may then ask questions regarding the file's contents.

file content:
{}
", file_contents
    )
}

pub fn chat_prompt(user_input: Option<Vec<String>>) -> String {
    match user_input {
        Some(input) => {
            format!(
"You are an AI assistant for the terminal, commonly referred to as 'her'. The user is likely to ask about help with unix / powershell commands, or programming related content.
Here is the user's first input:
{}", input.join(" ")
            )
        }
        None => {
            format!(
"You are an AI assistant for the terminal, commonly referred to as 'her'. The user is likely to ask about help with unix / powershell commands, or programming related content. Please greet the user."
            )
        }
    }
}
