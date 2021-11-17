
use builtin;
use str;

set edit:completion:arg-completer[git-ignore] = [@words]{
    fn spaces [n]{
        builtin:repeat $n ' ' | str:join ''
    }
    fn cand [text desc]{
        edit:complex-candidate $text &display=$text' '(spaces (- 14 (wcswidth $text)))$desc
    }
    var command = 'git-ignore'
    for word $words[1..-1] {
        if (str:has-prefix $word '-') {
            break
        }
        set command = $command';'$word
    }
    var completions = [
        &'git-ignore'= {
            cand -h 'Print help information'
            cand --help 'Print help information'
            cand -V 'Print version information'
            cand --version 'Print version information'
            cand -l 'List <templates> or all available templates'
            cand --list 'List <templates> or all available templates'
            cand -u 'Update templates by fetching them from gitignore.io'
            cand --update 'Update templates by fetching them from gitignore.io'
            cand -s 'Ignore all user defined aliases and templates'
            cand --simple 'Ignore all user defined aliases and templates'
            cand alias 'Manage local aliases'
            cand template 'Manage local templates'
            cand init 'Initialize user configuration'
            cand completion 'Generate shell completion'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'git-ignore;alias'= {
            cand -h 'Print help information'
            cand --help 'Print help information'
            cand list 'List available aliases'
            cand add 'Add a new alias'
            cand remove 'Remove an alias'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'git-ignore;alias;list'= {
            cand --version 'Print version information'
            cand -h 'Print help information'
            cand --help 'Print help information'
        }
        &'git-ignore;alias;add'= {
            cand --version 'Print version information'
            cand -h 'Print help information'
            cand --help 'Print help information'
        }
        &'git-ignore;alias;remove'= {
            cand --version 'Print version information'
            cand -h 'Print help information'
            cand --help 'Print help information'
        }
        &'git-ignore;alias;help'= {
            cand --version 'Print version information'
            cand -h 'Print help information'
            cand --help 'Print help information'
        }
        &'git-ignore;template'= {
            cand -h 'Print help information'
            cand --help 'Print help information'
            cand list 'List available templates'
            cand add 'Add a new template'
            cand remove 'Remove a template'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'git-ignore;template;list'= {
            cand --version 'Print version information'
            cand -h 'Print help information'
            cand --help 'Print help information'
        }
        &'git-ignore;template;add'= {
            cand --version 'Print version information'
            cand -h 'Print help information'
            cand --help 'Print help information'
        }
        &'git-ignore;template;remove'= {
            cand --version 'Print version information'
            cand -h 'Print help information'
            cand --help 'Print help information'
        }
        &'git-ignore;template;help'= {
            cand --version 'Print version information'
            cand -h 'Print help information'
            cand --help 'Print help information'
        }
        &'git-ignore;init'= {
            cand --force 'Forcefully create config, possibly overwrite existing'
            cand -h 'Print help information'
            cand --help 'Print help information'
        }
        &'git-ignore;completion'= {
            cand -h 'Print help information'
            cand --help 'Print help information'
        }
        &'git-ignore;help'= {
            cand -h 'Print help information'
            cand --help 'Print help information'
        }
    ]
    $completions[$command]
}
