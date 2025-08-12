import { FormEvent, useRef, useState } from 'react';
import Autosuggest, { RenderSuggestionParams } from 'react-autosuggest';
import { CategoryRepository } from '../hooks/useCategories';
import { GroceryListRepository } from '../hooks/useGroceryList';
import Executor from '../utils/cmd/exec';
import { parseInput } from '../utils/cmd/parser';

const getSuggestionValue = (suggestion: string) => suggestion.trim();

const renderSuggestion = (
    suggestion: string,
    { isHighlighted }: RenderSuggestionParams,
) => {
    if (isHighlighted) {
        return (
            <div style={{ background: 'red' }}>
                {suggestion}
            </div>
        )
    }

    return (
        <div>
            {suggestion}
        </div>
    )
};

interface ConsoleProps {
    groceryListRepository: GroceryListRepository,
    categoryRepository: CategoryRepository,
    onExecuteError: (e: Error) => void,
}

export default function Console({
    groceryListRepository,
    categoryRepository,
    onExecuteError,
}: ConsoleProps) {
    const [value, setValue] = useState('');
    const [suggestions, setSuggestions] = useState<string[]>([]);
    const inputRef = useRef(null);
    const autosuggestRef = useRef(null);
    const executor = new Executor(groceryListRepository, categoryRepository)

    const onSuggestionsFetchRequested = async (
        { value }: { value: string },
    ) => {
        if (value.length === 0) {
            setSuggestions([]);
        } else if (value[0] != '/') {
            const suggestions = await groceryListRepository.fetchSuggestions(value);
            setSuggestions(suggestions);
        } else {
            setSuggestions([
                '/help',
                '/category add',
                '/category rename',
                '/category remove',
            ].filter((suggestion) => suggestion.startsWith(value)))
        }
    }

    const handleSubmit = async (inputValue = value) => {
        if (!inputValue.trim()) return;

        const parseResult = parseInput(inputValue);

        const error = await executor.execute(parseResult);
        if (error === undefined) {
            setValue('');
            setSuggestions([]);
        } else {
            onExecuteError(error);
        }
    };

    const onKeyDown = (event: KeyboardEvent) => {
        if (event.ctrlKey && event.key === 'n') {
            // Prevent the default browser behavior (like opening a new window)
            event.preventDefault();
            const downArrowEvent = new KeyboardEvent('keydown', {
                key: 'ArrowDown',
                code: 'ArrowDown',
                keyCode: 40,
                which: 40,
                bubbles: true,
                cancelable: true
            });

            // Dispatch the event to the input element
            // @ts-ignore
            inputRef.current?.dispatchEvent(downArrowEvent);
        } else if (event.ctrlKey && event.key === 'p') {
            // Prevent the default browser behavior (like opening a new window)
            event.preventDefault();
            const upArrowEvent = new KeyboardEvent('keydown', {
                key: 'ArrowUp',
                code: 'ArrowUp',
                keyCode: 38,
                which: 38,
                bubbles: true,
                cancelable: true
            });

            // Dispatch the event to the input element
            // @ts-ignore
            inputRef.current?.dispatchEvent(upArrowEvent);
        }
        else if (event.key === 'Enter') {
            event.preventDefault();
            handleSubmit();
        } else if (event.key === 'Escape') {
            setValue('');
            setSuggestions([]);
            // @ts-ignore
            inputRef.current?.blur();
        }
    };

    const inputProps = {
        placeholder: "Add grocery item or type / for commands...",
        value,
        onChange: (_event: FormEvent, { newValue }: any) => setValue(newValue.toLowerCase()),
        onBlur: () => setSuggestions([]),
        onKeyDown: onKeyDown,
        ref: inputRef
    };

    return (
        <div className="relative">
            <Autosuggest
                ref={autosuggestRef}
                suggestions={suggestions}
                onSuggestionsFetchRequested={onSuggestionsFetchRequested}
                alwaysRenderSuggestions={true}
                shouldRenderSuggestions={() => true}
                getSuggestionValue={getSuggestionValue}
                renderSuggestion={renderSuggestion}
                // @ts-ignore
                inputProps={inputProps}
            />
        </div>
    )
}
