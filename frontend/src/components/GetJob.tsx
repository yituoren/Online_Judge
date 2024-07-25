import React, { useState } from 'react';
import jobService from '../services/jobService';
import './GetJob.css';

//response
interface Job {
    id: number,
    created_time: string,
    updated_time: string,
    submission: {
        source_code: string,
        language: string,
        user_id: number,
        contest_id: number,
        problem_id: number,
    },
    state: string,
    result: string,
    score: string,
    cases: {
        id: number,
        result: string,
        time: number,
        memory: number,
        info: string,
    }[],
}

const GetJob: React.FC = () => {
    //1 for joblist query, 2 for job by id
    const [responseMessage1, setResponseMessage1] = useState<Job[]>([]);
    const [responseMessage2, setResponseMessage2] = useState<Job | null>(null);
    const [error1, setError1] = useState<any>(null);
    const [error2, setError2] = useState<any>(null);

    const [id, setId] = useState<string>('');
    const [query, setQuery] = useState({
        user_id: '',
        user_name: '',
        contest_id: '',
        problem_id: '',
        language: '',
        from: '',
        to: '',
        state: '',
        result: '',
    })

    //functions for click and input
    const handleSubmit1 = async (e: React.FormEvent) => {
        e.preventDefault();
        try {
            const response: Job[] = await jobService.getJobs(query);
            setError1(null);
            setResponseMessage1(response);
        } catch (error: any) {
            setResponseMessage1([]);
            if (error.response && error.response.data) {
              setError1(error.response.data);
            } else {
              setError1({ error: 'An unknown error occurred' });
            }
        }
    }

    const handleSubmit2 = async (e: React.FormEvent) => {
        e.preventDefault();
        try {
            const response: Job = await jobService.getJobById(Number(id));
            setError2(null);
            setResponseMessage2(response);
        } catch (error: any) {
            setResponseMessage2(null);
            if (error.response && error.response.data) {
              setError2(error.response.data);
            } else {
              setError2({ error: 'An unknown error occurred' });
            }
        }
    }

    const handleInputChange2 = (e: React.ChangeEvent<HTMLInputElement>) => {
        setId(e.target.value);
    }

    const handleInputChange1 = (e: React.ChangeEvent<HTMLInputElement>) => {
        const { name, value } = e.target;
        setQuery(prevQuery => ({
            ...prevQuery,
            [name]: value,
        }));
    }

    //response component
    const renderJob = (job: Job) => {
        return (
            <div className="response-message">
                <div className="left-column">
                    <div className="response-row">
                        <div className="response-key">ID: </div>
                        <div className="response-value">{job.id}</div>
                    </div>
                    <div className="response-row">
                        <div className="response-key">Created: </div>
                        <div className="response-value">{job.created_time}</div>
                    </div>
                    <div className="response-row">
                        <div className="response-key">Updated: </div>
                        <div className="response-value">{job.updated_time}</div>
                    </div>
                    <div className="response-row">
                        <div className="response-key">Submission: </div>
                            <div className="response-value">
                                {Object.entries(job.submission).map(key => (
                                    <div>
                                        <strong>{key[0]}:</strong> {key[1]}
                                    </div>
                                ))}
                            </div>
                        </div>
                    <div className="response-row">
                        <div className="response-key">State: </div>
                        <div className="response-value">{job.state}</div>
                    </div>
                </div>
                <div className="right-column">
                    <div className="response-row">
                        <div className="response-key">Score: </div>
                        <div className="response-value">{job.score}</div>
                    </div>
                    {job.cases && job.cases.map((Case, index) => (
                        <div className="response-row">
                            <div className="response-key">Case {index + 1}:</div>
                            <div className="response-value">
                                {Object.entries(Case).map(key => (
                                    <div>
                                        <strong>{key[0]}:</strong> {key[1]}
                                    </div>
                                ))}
                            </div>
                        </div>
                    ))}
                </div>
            </div>
        )
    }

    //error component
    const renderError = (error: any) => {
        return (
            <div className="response-message">
                <div className="left-column">
                {Object.entries(error)
                    .map(([key, value], index) => (
                    <div className="response-row" key={index}>
                        <div className="response-key">{key}:</div>
                        <div className="response-value">{JSON.stringify(value, null, 2)}</div>
                    </div>
                    ))}
                </div>
            </div>
        )
    }

    return (
        <div className='get-page'>
            <div className='form-container'>
            <div className='left-col'>
                <form className='get-form' onSubmit={handleSubmit1}>
                    <input
                        type='number'
                        name='user_id'
                        value={query.user_id}
                        onChange={handleInputChange1}
                        placeholder='User ID'
                    />
                    <input
                        type='number'
                        name='contest_id'
                        value={query.contest_id}
                        onChange={handleInputChange1}
                        placeholder='Contest ID'
                    />
                    <input
                        type='number'
                        name='problem_id'
                        value={query.problem_id}
                        onChange={handleInputChange1}
                        placeholder='Problem ID'
                    />
                    <input
                        type='string'
                        name='user_name'
                        value={query.user_name}
                        onChange={handleInputChange1}
                        placeholder='User Name'
                    />
                    <input
                        type='string'
                        name='from'
                        value={query.from}
                        onChange={handleInputChange1}
                        placeholder='From'
                    />
                    <input
                        type='string'
                        name='to'
                        value={query.to}
                        onChange={handleInputChange1}
                        placeholder='To'
                    />
                    <input
                        type='string'
                        name='language'
                        value={query.language}
                        onChange={handleInputChange1}
                        placeholder='Language ID'
                    />
                    <input
                        type='string'
                        name='state'
                        value={query.state}
                        onChange={handleInputChange1}
                        placeholder='State'
                    />
                    <input
                        type='string'
                        name='result'
                        value={query.result}
                        onChange={handleInputChange1}
                        placeholder='Result'
                    />
                    <button type='submit'>Search</button>
                    {responseMessage1 && responseMessage1.map(job => renderJob(job))}
                    {error1 && renderError(error1)}
                </form>
            </div>
            <div className='right-col'>
                <form className='get-id' onSubmit={handleSubmit2}>
                    <input
                        type="number"
                        name="id"
                        value={id}
                        onChange={handleInputChange2}
                        placeholder='Job ID'
                    />
                    <button type='submit'>Get</button>
                    {responseMessage2 && renderJob(responseMessage2)}
                    {error2 && renderError(error2)}
                </form>
            </div>
            </div>
        </div>
    );
}

export default GetJob;