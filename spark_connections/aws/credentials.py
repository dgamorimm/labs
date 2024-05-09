import boto3


class AwsStsCredentials:
    def __init__(self, region_name : str):
        self.client = boto3.client('sts',region_name=region_name)

    def get(self) -> str:
        response = self.client.get_session_token()
        access_key_id = response['Credentials']['AccessKeyId']
        secret_access_key = response['Credentials']['SecretAccessKey']
        session_token = response['Credentials']['SessionToken']
        
        return access_key_id, secret_access_key, session_token