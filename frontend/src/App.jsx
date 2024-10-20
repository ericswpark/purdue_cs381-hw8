import React, { useState } from 'react';
import { createRoot } from 'react-dom/client';

import InputField from './components/InputField';

const SERVER_BASE_URL = "https://purdue-cs381-api.ericswpark.com";

async function fetch_question_answer(num, body) {
    let res = await fetch(`${SERVER_BASE_URL}/${num}`, {
        method: "POST",
        headers: {
            "Content-Type": "application/json"
        },
        body: JSON.stringify(body)
    });
    return await res;
}


function Q2() {
    const [arrayP, setArrayP] = useState("8 2 4 5 9 6 3");
    const [arrayT, setArrayT] = useState("0 4 1 6 2 0 0");
    const [eVal, setEVal] = useState("5");
    const [status, setStatus] = useState(null);
    const [result, setResult] = useState(null);

    async function calculateValue(event) {
        event.preventDefault();
        setStatus("loading");

        let P = arrayP.split(" ").map(Number);
        let T = arrayT.split(" ").map(Number);
        let E = Number(eVal);

        let body = {
            p: P,
            t: T,
            e: E
        };

        try {
            let res = await fetch_question_answer(2, body);

            try {
                let res_json = await res.clone().json();

                if (res_json && res_json.hasOwnProperty("answer")) {
                    setResult(res_json.answer);
                    setStatus("success");
                    return;
                } else if (res_json && res_json.hasOwnProperty("error")) {
                    setResult(res_json.error);
                } else {
                    setResult("Failed to retrieve answer.")
                }
            } catch (e) {
                setResult(await res.text())
            }
            setStatus("error");
        } catch (e) {
            setResult(e.message);
            setStatus("error");
        }
    }


    return (
        <>
            <form onSubmit={calculateValue}>
                <InputField value={arrayP} setValue={setArrayP} label={"Array P (separated by spaces): "} type={"text"} />
                <InputField value={arrayT} setValue={setArrayT} label={"Array T (separated by spaces): "} type={"text"} />
                <InputField value={eVal} setValue={setEVal} label={"(End-of-term) E: "} type={"number"} min={0} max={65535}/>
                <input type="submit" value='Calculate' disabled={arrayP.length === 0 || arrayT.length === 0 || eVal.length === 0 || setStatus === "loading"}></input>
            </form>

            <div>
                {status === "loading" && <h4>Loading...</h4>}
                {status === "error" && <h4>Error: {result ?? ""}</h4>}
                {status === "success" && <h4>Answer: {result ?? ""}</h4>}
            </div>
        </>
    );
}

const q2_container = createRoot(document.getElementById("q2-container"));
q2_container.render(<Q2 />)