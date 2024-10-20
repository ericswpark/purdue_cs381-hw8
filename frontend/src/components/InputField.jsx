import React, { useId } from 'react';

function InputField({ value, setValue, label, ...params }) {
    const id = useId();
    return (
        <>
            <label htmlFor={id}><strong>{label}</strong></label>
            <input id={id} value={value} onChange={e => setValue(e.target.value)} {...params} /><br /><br />
        </>
    )
}

export default InputField;