import { useState } from 'react';
import Autosuggest from 'react-autosuggest';

const getSuggestionValue = suggestion => suggestion;

const renderSuggestion = suggestion => (
    <div>
        {suggestion}
    </div>
);

export default function Console(
) {
    const [value, setValue] = useState('');
    const [suggestions, setSuggestions] = useState([]);

    const inputProps = {
        placeholder: "Add grocery item or type / for commands...",
        value,
        onChange: (event, { newValue }) => setValue(newValue),
        // onKeyDown,
        // ref: inputRef,
    };

    const onSuggestionsFetchRequested = ({ value }) => {
        setSuggestions([
            '/test',
            '/category add',
            '/category rename',
            '/category remove',
            'black beans',
        ])
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
