# coding: utf-8

import requests
import json
import io
import time
import os.path
import sys
import argparse

global DEFAULT_COOLDOWN, DEFAULT_API_KEY_PATH, DEFAULT_REALM, DEFAULT_TYPE
DEFAULT_COOLDOWN = 300
DEFAULT_API_KEY_PATH = '/home/uj/dev/apikeys/lol'
DEFAULT_REALM = "EUW"
DEFAULT_TYPE = 'featured'

class Api(object):

    def __init__(self):
        self.counter = 0

    def get(self, url):
        self.counter += 1
        try:
            return requests.get(url).json()
        except Exception as  e:
            raise e

class Response(object):

    def __init__(self, status, content_type, cooldown, content):
        self.status = status
        self.content_type = content_type
        self.cooldown = cooldown
        self.content = content

    def send(self, out=sys.stdout):
         out.write(str(json.dumps(self.__dict__)))


def forge_request(api_key, realm=DEFAULT_REALM, type=DEFAULT_TYPE):
    # TODO: This function should take some parameters and forge a url.
    # from get_featured.py
    # Should be https://" + realm + ".api.pvp.net/" + ? + ... and so on.

    # if type == 'static':
    #     return "https://global.api.pvp.net/api/lol/static-data/na/v1.2/champion?dataById=true&api_key=" + api_key
    return "https://"+ realm + ".api.pvp.net/observer-mode/rest/featured?api_key=" + api_key

#TODO: EAFP
def extract_nicknames(json_data):
    nicknames = {}

    if 'gameList' not in json_data:
        raise ValueError('No gamelist given.', json_data)

    for game in json_data['gameList']:
        if game['gameMode'] == "CLASSIC" and game['gameType'] == "MATCHED_GAME":
            for participant in game['participants']:
                # This thing is compactable like blablabla.append(blablabl)  if participant in game['participants'] else blablabla
                if participant['championId'] not in nicknames:
                    nicknames[participant['championId']] = [participant['summonerName']]
                else:
                    nicknames[participant['championId']].append(participant['summonerName'])

    return nicknames

def get_cooldown(json_data):
    if 'clientRefreshInterval' not in json_data:
        return DEFAULT_COOLDOWN
    return json_data['clientRefreshInterval']

def get_api_key(path=DEFAULT_API_KEY_PATH):
    file = open(path, 'r')
    return file.readline().strip()

def init_argparse():
    parser = argparse.ArgumentParser(description='Twitch : nickname and static data getter')
    parser.add_argument('-r', '--realm', type=str, action="store", nargs='?', const="euw",
                        help='World region to poke : EUW, NA, EUNE, etc.\n Default: euw')
    # TODO: Document this.
    parser.add_argument('-t', '--type', type=str, action="store",
                        help='Type of data to get : static, featured, mastery\n Default: featured')

    parser.add_argument('-f', '--force-cooldown', type=int, action="store",
                        help='Force the script to return a defined cooldown value')

    parser.add_argument('-a', '--api-file', type=str, action="store",
                        help='Specify a file where the API key is\n Default:' + DEFAULT_API_KEY_PATH)
    parser.add_argument('-c', '--custom-url', type=str, action="store",
                        help='Query a specific url. May override -t, -f, -a.')

    result = parser.parse_args()
    return dict(result._get_kwargs())


if __name__ == "__main__":

    api = Api()
    args = init_argparse()
    try:

        if args['api_file'] is not None:
            api_key = get_api_key(args['api_file'])
        else:
            api_key = get_api_key()
        if args['custom_url'] is not None:
            api_response = api.get(args['custom_url'])
        else:
            #Forging request
            if args['realm'] is not None:
                api_response = api.get(forge_request(api_key, args['realm']))
            else:
                api_response = api.get(forge_request(api_key))
        if args['force_cooldown'] is not None:
            cooldown = args['force_cooldown']
        else:
            cooldown = get_cooldown(api_response)
        content = extract_nicknames(api_response)
        content_type = 'nicknames'
        status = 0
    except Exception as  e:
        cooldown = DEFAULT_COOLDOWN
        content = str(e)
        content_type = 'error'
        status = 1
    response = Response(status, content_type, cooldown, content)
    response.send()
