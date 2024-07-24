import axios from 'axios';

const api = axios.create({
  baseURL: 'http://localhost:12345', // 后端服务器地址
});

export default api;