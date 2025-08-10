import { useRef, useState } from 'react';
import Autosuggest from 'react-autosuggest';
import { GroceryListRepository } from '../hooks/useGroceryList';

const getSuggestionValue = suggestion => suggestion.trim();

const renderSuggestion = (suggestion, { query, isHighlighted }) => {
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
    groceryListRepository: GroceryListRepository
}

export default function Console({
    groceryListRepository,
}: ConsoleProps) {
    const [value, setValue] = useState('');
    const [suggestions, setSuggestions] = useState<string[]>([]);
    const inputRef = useRef(HTMLInputElement);
    const autosuggestRef = useRef(null);

    const onSuggestionsFetchRequested = async ({ value }) => {
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
            ])
        }
    }

    const parseInput = (inputValue) => {
        if (!inputValue.startsWith('/')) {
            return { type: 'item', value: inputValue };
        }

        const parts = inputValue.split(' ');
        const commandPart = parts[0];

        if (inputValue.endsWith(' ') || parts.length > 1) {
            // Command is complete, now in parameter mode
            if (commandPart === '/category' && parts[1] === 'add') {
                return {
                    type: 'category-name',
                    value: parts.slice(2).join(' '),
                    command: '/category add'
                };
            }
        }

        return {
            type: 'command',
            value: inputValue
        };
    };

    const handleSubmit = (inputValue = value) => {
        if (!inputValue.trim()) return;

        const context = parseInput(inputValue);

        if (context.type === 'item') {
            groceryListRepository.createEntry(context.value, 0);
        }

        setValue('');
        setSuggestions([]);
    };

    const onKeyDown = (event) => {
        if (event.key === 'Enter') {
            event.preventDefault();
            handleSubmit();
        } else if (event.key === 'Escape') {
            setValue('');
            setSuggestions([]);
            inputRef.current?.blur();
        }
    };

    const inputProps = {
        placeholder: "Add grocery item or type / for commands...",
        value,
        onChange: (event, { newValue }) => setValue(newValue),
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
                inputProps={inputProps}
            />
        </div>
    )
}
