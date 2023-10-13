import requests
import unittest


class HealthCheckEndpointTests(unittest.TestCase):
    def test_v1_health_check_get(self):
        api_url = "http://localhost:9000/api/v1/health_check"
        response = requests.get(api_url)
        self.assertTrue(response.status_code == 200)

    def test_v1_health_check_post(self):
        api_url = "http://localhost:9000/api/v1/health_check"
        response = requests.post(api_url)
        self.assertTrue(response.status_code == 404)

    def test_v1_health_check_put(self):
        api_url = "http://localhost:9000/api/v1/health_check"
        response = requests.put(api_url)
        self.assertTrue(response.status_code == 404)


if __name__ == '__main__':
    unittest.main()
