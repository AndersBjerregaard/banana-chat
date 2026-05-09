import http from 'k6/http';
import { sleep } from 'k6';
import { Options } from 'k6/options';

// Adding the 'Options' type gives you autocomplete for k6 settings
export const options: Options = {
  vus: 10,
  duration: '30s',
};

export default function () {
  http.get('http://test.k6.io');
  sleep(1);
}
