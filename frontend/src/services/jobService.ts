import api from './api';

//actually this file contains all the services, not only for jobs
interface Job {
    source_code: string;
    language: string;
    user_id: number;
    contest_id: number;
    problem_id: number;
  }

const createJob = async (job: Job) => {
  try {
    const jsonJob = JSON.stringify(job);
    const response = await api.post('/jobs', jsonJob, {
      headers: {
        'Content-Type': 'application/json'
      }
    });
    
    return response.data;
  } catch (error: unknown) {
    console.error('Error creating job:', error);
    throw error;
  }
};
  
const getJobs = async (query: Record<string, string>) => {
  try {
    const params = new URLSearchParams();
    Object.entries(query).forEach(([key,value]) => {
      if (value) {
        params.append(key, value);
      }
    })
    const response = await api.get(`/jobs?${params}`);
    return response.data;
  } catch (error) {
    console.error('Error fetching jobs:', error);
    throw error;
  }
};
  
const getJobById = async (id: number) => {
  try {
    const response = await api.get(`/jobs/${id}`);
    return response.data;
  } catch (error) {
    console.error('Error fetching job by id:', error);
    throw error;
  }
};
  
const updateJobById = async (id: number) => {
  try {
    const response = await api.put(`/jobs/${id}`);
    return response.data;
  } catch (error) {
    console.error('Error updating job by id:', error);
    throw error;
  }
};
  
const deleteJobById = async (id: number) => {
  try {
    const response = await api.delete(`/jobs/${id}`);
    return response.data;
  } catch (error) {
    console.error('Error deleting job by id:', error);
    throw error;
  }
};

const getContestRank = async (id: number, query: Record<string, string>) => {
  try {
    const params = new URLSearchParams();
    Object.entries(query).forEach(([key,value]) => {
      if (value) {
        params.append(key, value);
      }
    })
    const response = await api.get(`/contests/${id}/ranklist?${params}`);
    return response.data;
  } catch (error) {
    console.error('Error fetching jobs:', error);
    throw error;
  }
};

const getContest = async (id: number) => {
  if (id === 0) {
    try {
      const response = await api.get(`/contests`);
      return response.data;
    } catch (error) {
      console.error('Error fetching job by id:', error);
      throw error;
    }
  }
  else {
    try {
      const response = await api.get(`/contests/${id}`);
      return response.data;
    } catch (error) {
      console.error('Error fetching job by id:', error);
      throw error;
    }
  }
};
  
export default {
  createJob,
  getJobs,
  getJobById,
  updateJobById,
  deleteJobById,
  getContestRank,
  getContest,
};