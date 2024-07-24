import api from './api';

interface Job {
    source_code: string;
    language: string;
    user_id: number;
    contest_id: number;
    problem_id: number;
  }

const createJob = async (job: Job) => {
  try {
    // 将 Job 对象序列化成 JSON 字符串
    const jsonJob = JSON.stringify(job);
  
    // 发送 POST 请求，并将 JSON 字符串作为请求体
    const response = await api.post('/jobs', jsonJob, {
      headers: {
        'Content-Type': 'application/json' // 确保请求头中指定内容类型为 JSON
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