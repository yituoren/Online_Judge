import axios from 'axios';

//create api
const api = axios.create({
  baseURL: 'http://localhost:12345',
});

export default api;