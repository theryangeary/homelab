import { useState } from 'react';
import Autosuggest from 'react-autosuggest';
import { GroceryListRepository } from '../hooks/useGroceryList';

const getSuggestionValue = suggestion => suggestion;

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

    const inputProps = {
        placeholder: "Add grocery item or type / for commands...",
        value,
        onChange: (event, { newValue }) => setValue(newValue),
        // onKeyDown,
        // ref: inputRef,
        onBlur: () => setSuggestions([]),
    };

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

    return (
        <div className="relative">
            <Autosuggest
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
