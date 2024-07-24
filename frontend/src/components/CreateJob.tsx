import React, { useState } from 'react';
import jobService from '../services/jobService';
import './CreateJob.css'

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

const CreateJob: React.FC = () => {
  const [job, setJob] = useState({
    source_code: '',
    language: '',
    user_id: '',
    contest_id: '',
    problem_id: '',
  });

  const [responseMessage, setResponseMessage] = useState<Job | null>(null);
  const [error, setError] = useState<any>(null);

  const handleInputChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const { name, value } = e.target;
    setJob(prevJob => ({
      ...prevJob,
      [name]: value,
    }));
  };

  const handleTextareaChange = (e: React.ChangeEvent<HTMLTextAreaElement>) => {
    const { name, value } = e.target;
    setJob(prevJob => ({
      ...prevJob,
      [name]: value,
    }));
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    try {
      const response: Job = await jobService.createJob({
        ...job,
        user_id: Number(job.user_id),
        contest_id: Number(job.contest_id),
        problem_id: Number(job.problem_id),
      });
      setError(null);
      setResponseMessage(response);
    } catch (error: any) {
      setResponseMessage(null);
      if (error.response && error.response.data) {
        setError(error.response.data);
      } else {
        setError({ error: 'An unknown error occurred' });
      }
    }
  };

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
                                    <strong>{key[0]}:</strong> {key[1]};
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
                                    <strong>{key[0]}:</strong> {key[1]};
                                </div>
                            ))}
                        </div>
                    </div>
                ))}
            </div>
        </div>
    )
}

  return (
    <div className="home-page">
      <form className="job-form" onSubmit={handleSubmit}>
        <div className="input-column">
          <input
            type="text"
            name="language"
            value={job.language}
            onChange={handleInputChange}
            placeholder="Language"
          />
          <input
            type="number"
            name="user_id"
            value={job.user_id}
            onChange={handleInputChange}
            placeholder="User ID"
          />
          <input
            type="number"
            name="contest_id"
            value={job.contest_id}
            onChange={handleInputChange}
            placeholder="Contest ID"
          />
          <input
            type="number"
            name="problem_id"
            value={job.problem_id}
            onChange={handleInputChange}
            placeholder="Problem ID"
          />
          <button type="submit">Submit</button>
        </div>
        <textarea
          name="source_code"
          value={job.source_code}
          onChange={handleTextareaChange}
          placeholder="Source Code"
        />
      </form>
      {responseMessage && renderJob(responseMessage)}
      {error && reportError(error)}
    </div>
  );
};

export default CreateJob;
