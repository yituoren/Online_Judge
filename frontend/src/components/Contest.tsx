import React, { useState } from 'react';
import jobService from '../services/jobService';
import './Contest.css';

interface Contest {
    id: number,
    name: string,
    from: string,
    to: string,
    problem_ids: number[],
    user_ids: number[],
    submission_limit: number,
}

interface UserRank {
    user: {
        id: number,
        name: string,
    },
    rank: number,
    scores: number[],
}

const GetContest: React.FC = () => {
    //1 for contestlist query, 3 for contest by id, 2 for rank
    const [responseMessage1, setResponseMessage1] = useState<Contest[]>([]);
    const [responseMessage3, setResponseMessage3] = useState<Contest | null>(null);
    const [responseMessage2, setResponseMessage2] = useState<UserRank[]>([]);
    const [error1, setError1] = useState<any>(null);
    const [error2, setError2] = useState<any>(null);

    const [id1, setId1] = useState<string>('');
    const [id2, setId2] = useState<string>('');
    const [query, setQuery] = useState({
        scoring_rule: 'latest',
        tie_breaker: '',
    })

    const handleInputChange1 = async (e: React.ChangeEvent<HTMLInputElement>) => {
        setId1(e.target.value);
    }

    const handleInputChange2 = async (e: React.ChangeEvent<HTMLInputElement>) => {
        setId2(e.target.value);
    }

    //selection change
    const handleInputChange3 = async (e: React.ChangeEvent<HTMLSelectElement>) => {
        const { name, value } = e.target;
        setQuery(prevQuery => ({
        ...prevQuery,
        [name]: value,
        }));
    }

    const handleSubmit1 = async (e: React.FormEvent) => {
        e.preventDefault();
        try {
            const response = await jobService.getContest(Number(id1));
            setError1(null);
            if (Number(id1) === 0) {
                setResponseMessage3(null);
                const contest: Contest[] = response;
                setResponseMessage1(contest);
            }
            else {
                setResponseMessage1([]);
                const contest: Contest = response;
                setResponseMessage3(contest);
            }
        } catch (error: any) {
            setResponseMessage1([]);
            setResponseMessage3(null);
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
            const response = await jobService.getContestRank(Number(id2), query);
            setError2(null);
            const rank: UserRank[] = response;
            setResponseMessage2(rank);
        } catch (error: any) {
            setResponseMessage2([]);
            if (error.response && error.response.data) {
                setError2(error.response.data);
            } else {
                setError2({ error: 'An unknown error occurred' });
            }
        }
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

    //response component
    const renderContest = (contest: Contest) => {
        return (
            <div className="response-message">
                <div className="left-column">
                    <div className="response-row">
                        <div className="response-key">ID: </div>
                        <div className="response-value">{contest.id}</div>
                    </div>
                    <div className="response-row">
                        <div className="response-key">From: </div>
                        <div className="response-value">{contest.from}</div>
                    </div>
                    <div className="response-row">
                        <div className="response-key">Users: </div>
                        <div className="response-value">{contest.user_ids}</div>
                    </div>
                    <div className="response-row">
                        <div className="response-key">Sub Limit: </div>
                        <div className="response-value">{contest.submission_limit}</div>
                    </div>
                </div>
                <div className="right-column">
                    <div className="response-row">
                        <div className="response-key">Name: </div>
                        <div className="response-value">{contest.name}</div>
                    </div>
                    <div className="response-row">
                        <div className="response-key">To: </div>
                        <div className="response-value">{contest.to}</div>
                    </div>
                    <div className="response-row">
                        <div className="response-key">Problems: </div>
                        <div className="response-value">{contest.problem_ids}</div>
                    </div>
                </div>
            </div>
        )
    }

    const renderRank = (rank: UserRank) => {
        return (
            <div className="response-message">
                <div className="left-column">
                    <div className="response-row">
                        <div className="response-key">ID: </div>
                        <div className="response-value">{rank.user.id}</div>
                    </div>
                    <div className="response-row">
                        <div className="response-key">Rank:</div>
                        <div className="response-value">{rank.rank}</div>
                    </div>
                </div>
                <div className="right-column">
                    <div className="response-row">
                        <div className="response-key">Name:</div>
                        <div className="response-value">{rank.user.name}</div>
                    </div>
                    <div className="response-row">
                        <div className="response-key">Scores:</div>
                        <div className="response-value">
                                {rank.scores.map(key => (
                                    <div>{key}</div>
                                ))}
                        </div>
                    </div>
                </div>
            </div>
        )
    }

    return(
        <div className='contest-page'>
        <div className='form-container'>
        <div className='left-col'>
            <form className='contest-form' onSubmit={handleSubmit1}>
                <input
                    type='number'
                    name='contest_id'
                    value={id1}
                    onChange={handleInputChange1}
                    placeholder='Contest ID'
                />
                <button type='submit'>Get Info</button>
                {responseMessage1 && responseMessage1.map(contest => renderContest(contest))}
                {responseMessage3 && renderContest(responseMessage3)}
                {error1 && renderError(error1)}
            </form>
        </div>
        <div className='right-col'>
            <form className='contest-id' onSubmit={handleSubmit2}>
                <input
                    type='number'
                    name='contest_id'
                    value={id2}
                    onChange={handleInputChange2}
                    placeholder='Contest ID'
                />
                <select
                    name='scoring_rule'
                    value={query.scoring_rule}
                    onChange={handleInputChange3}
                >
                    <option value='latest'>latest</option>
                    <option value='highest'>highest</option>
                </select>
                <select
                    name='tie_breaker'
                    value={query.tie_breaker}
                    onChange={handleInputChange3}
                >
                    <option value=''>null</option>
                    <option value='submission_time'>submission time</option>
                    <option value='submission_count'>submission count</option>
                    <option value='user_id'>user id</option>
                </select>
                <button type='submit'>Get Rank</button>
                {responseMessage2 && responseMessage2.map(rank => renderRank(rank))}
                {error2 && renderError(error2)}
            </form>
        </div>
        </div>
    </div>
    );
}

export default GetContest