// Define the enum for command kinds
export enum CommandKind {
  Item = 'item',
  Help = 'help',
  CategoryAdd = 'category-add',
  CategoryRename = 'category-rename',
  CategoryRemove = 'category-remove',
  UnknownCommand = 'unknown-command',
}

// Define the discriminated union for all possible parse results
export type ParseResult = 
  | { kind: CommandKind.Item; value: string }
  | { kind: CommandKind.Help }
  | { kind: CommandKind.CategoryAdd; categoryName: string }
  | { kind: CommandKind.CategoryRename; oldName: string; newName: string }
  | { kind: CommandKind.CategoryRemove; categoryName: string }
  | { kind: CommandKind.UnknownCommand; command: string, errorString: string };

export const parseInput = (inputValue: string): ParseResult => {
  const trimmed = inputValue.trim();
  
  if (!trimmed.startsWith('/')) {
    return { kind: CommandKind.Item, value: trimmed };
  }

  const parts = trimmed.split(/\s+/); // Use regex to handle multiple spaces
  const command = parts[0];

  switch (command) {
    case '/help':
      return { kind: CommandKind.Help };
      
    case '/category':
      return parseCategoryCommand(parts.slice(1));
      
    default:
      return { kind: CommandKind.UnknownCommand, command: trimmed, errorString: `unknown command: ${command}` };
  }
};

const parseCategoryCommand = (args: string[]): ParseResult => {
  if (args.length === 0) {
    return { kind: CommandKind.UnknownCommand, command: '/category', errorString: "/category requires subcommand"};
  }

  const subcommand = args[0];
  
  switch (subcommand) {
    case 'add':
      const categoryName = args.slice(1).join(' ').trim();
      if (!categoryName) {
        return { kind: CommandKind.UnknownCommand, command: '/category add', errorString: "/category add requires the name of the category to add" };
      }
      return { kind: CommandKind.CategoryAdd, categoryName };
      
    case 'rename':
      if (args.length != 3) {
        return { kind: CommandKind.UnknownCommand, command: '/category rename', errorString: "/category rename takes exactly 2 arguments, oldName and newName"};
      }
      // Simple approach: assume first arg after 'rename' is old name, rest is new name
      // For more robust parsing, you might want to use quotes or a different delimiter
      const oldName = args[1];
      const newName = args[2];
      return { kind: CommandKind.CategoryRename, oldName, newName };
      
    case 'remove':
      const nameToRemove = args.slice(1).join(' ').trim();
      if (!nameToRemove) {
        return { kind: CommandKind.UnknownCommand, command: '/category remove', errorString: "/category remove requires the name of the category to remove" };
      }
      return { kind: CommandKind.CategoryRemove, categoryName: nameToRemove };
      
    default:
      return { kind: CommandKind.UnknownCommand, command: `/category ${subcommand}`, errorString: `unknown /category subcommand: ${subcommand}`};
  }
};

