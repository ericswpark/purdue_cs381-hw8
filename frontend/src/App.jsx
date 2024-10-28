import React, { useState, useEffect } from 'react';
import { createRoot } from 'react-dom/client';

import InputField from './components/InputField';

const SERVER_BASE_URL = "https://hw8.purdue-cs381-api.ericswpark.com";

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

async function fetch_question_test_cases(num) {
    let res = await fetch(`${SERVER_BASE_URL}/${num}/test_cases`);
    return await res.json();
}


function Q1() {
    const [arrayT, setArrayT] = useState("");
    const [arrayD, setArrayD] = useState("");
    const [status, setStatus] = useState(null);
    const [result, setResult] = useState(null);
    const [resultOrdering, setResultOrdering] = useState(null);
    const [resultProcessOrdering, setResultProcessOrdering] = useState(null);
    const [testCases, setTestCases] = useState([]);

    useEffect(() => {
        async function fetchAndSetTestCases() {
            const test_cases = await fetch_question_test_cases(1);
            setTestCases(test_cases);

            // Initialize first test case in input fields
            if (test_cases.length > 0) {
                setArrayT(test_cases[0].t.join(" "));
                setArrayD(test_cases[0].d.join(" "));
            }
        }
        fetchAndSetTestCases();
    }, [])

    async function calculateValue(event) {
        event.preventDefault();
        setStatus("loading");

        let T = arrayT.split(" ").map(Number);
        let D = arrayD.split(" ").map(Number);

        let body = {
            t: T,
            d: D
        };

        try {
            let res = await fetch_question_answer(1, body);

            try {
                let res_json = await res.clone().json();

                if (res_json && res_json.hasOwnProperty("answer") && res_json.hasOwnProperty("answer_ordering") && res_json.hasOwnProperty("answer_process_ordering")) {
                    setResult(res_json.answer);
                    setResultOrdering(res_json.answer_ordering.join(", "));
                    setResultProcessOrdering(res_json.answer_process_ordering.join(", "));
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

    function fillInTestCase(event) {
        let selected_test_case = testCases.find((test_case) => test_case.name === event.target.value);
        if (selected_test_case) {
            setArrayT(selected_test_case.t.join(" "));
            setArrayD(selected_test_case.d.join(" "));
        }
    }

    return (
        <>
            <h4 style={{display: "inline"}}>Select a test case: </h4>
            <select onChange={fillInTestCase}>
                {testCases.map((test_case) => <option key={test_case.name} value={test_case.name}>{test_case.name}</option>)}
            </select><br /><br />
            

            <form onSubmit={calculateValue}>
                <InputField value={arrayT} setValue={setArrayT} label={"Array T (separated by spaces): "} type={"text"} />
                <InputField value={arrayD} setValue={setArrayD} label={"Array D (separated by spaces): "} type={"text"} />
                <input type="submit" value='Calculate' disabled={arrayT.length === 0 || arrayD.length === 0 || setStatus === "loading"}></input>
            </form>

            <div>
                {status === "loading" && <h4>Loading...</h4>}
                {status === "error" && <h4>Error: {result ?? ""}</h4>}
                {status === "success" && <><h4>Answer: {result ?? ""}</h4>
                    <h5>Drink completion ordering: {resultOrdering ?? ""}</h5>
                    <h5>Drink process ordering: {resultProcessOrdering ?? ""}</h5>
                </>}
            </div>
        </>
    );
}

const q1_container = createRoot(document.getElementById("q1-container"));
q1_container.render(<Q1 />)

