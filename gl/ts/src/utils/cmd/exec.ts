import { CategoryRepository } from "../../hooks/useCategories";
import { GroceryListRepository } from "../../hooks/useGroceryList";
import { CommandKind, ParseResult } from "./parser";

export default class Executor {
    private glr;
    private cr;

    constructor(glr: GroceryListRepository, cr: CategoryRepository) {
        this.glr = glr;
        this.cr = cr;
    }

        async execute(pr: ParseResult): Promise<Error | undefined> {
        switch (pr.kind) {
            case CommandKind.Item:
                await this.glr.createEntry(pr.value, 0);
                break;

            case CommandKind.Help:
                return new Error('Available commands:\n<item> - Add an item to the grocery list\n/help - Show this help\n/category add <name> - Add a new category\n/category rename <oldName> <newName> - Rename a category\n/category remove <name> - Remove a category');

            case CommandKind.CategoryAdd:
                await this.cr.createCategory(pr.categoryName);
                break;

            case CommandKind.CategoryRename:
                const category = this.cr.categories.find(c => c.name === pr.oldName);
                if (!category) {
                    return new Error(`Category "${pr.oldName}" not found`);
                }
                await this.cr.updateCategory(category.id, { name: pr.newName });
                break;

            case CommandKind.CategoryRemove:
                const categoryToRemove = this.cr.categories.find(c => c.name === pr.categoryName);
                if (!categoryToRemove) {
                    return new Error(`Category "${pr.categoryName}" not found`);
                }
                await this.cr.deleteCategory(categoryToRemove.id);
                break;

            case CommandKind.UnknownCommand:
                return new Error(pr.errorString);

            default:
                // Exhaustive check - TypeScript will ensure this is never reached
                const _exhaustive: never = pr;
                return new Error(`Unhandled command kind: ${_exhaustive}`);
        }
    }
}
