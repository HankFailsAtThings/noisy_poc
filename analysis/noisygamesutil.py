import json
from os import listdir
import re

def load_matchup_files(players_struct,player,matchup):
    round_files = players_struct[player][matchup]
    rounds = []
    for rnd in round_files:
        f = open(rnd)
        rounds.append(json.load(f))
        f.close()
    return rounds

def build_players_struct(base_dir):
    # python regex are the dumbest thing ever implemented
    print(base_dir)   
    #first I need to figure out what all the folders are
    player_folders=listdir(base_dir)
    #from that, parse and figure out how many players there are
    indices = []
    for name in player_folders:
        pieces = name.split('player')
        indices.append(int(pieces[1]))
        indices.append(int(pieces[-1]))
    min_idx = min(indices)
    max_idx = max(indices)
    players = []
    for i in range(min_idx, max_idx+1):
        players.append('player'+str(i))

    #now with a list of players, if player is in player folder,
    #then it is to be included in the 
    players_struct = {};
    for player in players:
        players_struct[player] = {}
        for fldr in player_folders:
            #shitty hack
            is_in = False
            spl = fldr.split('p')
            for tok in spl:
                if player[1:] in tok and len(player[1:]) == len(tok):
                    is_in = True
            #end shitty hack 
            if is_in:
                round_folders = listdir(base_dir+fldr)
                players_struct[player][fldr] = []
                for rnd in round_folders:
                    players_struct[player][fldr].append(base_dir+fldr+'/'+rnd)
    return players_struct
def get_matchup_scores(rounds, player_name, matchup_name):
    #can check if last character matches, if it does, reverse, otherwise, normal scoring
    a_score = 0
    b_score = 0
    a_strat = list(rounds[0]['player_a'].keys())[0]
    b_strat = list(rounds[0]['player_b'].keys())[0]

    for rnd in rounds:
        a_score += rnd['player_a'][a_strat]['player']['play']['my_score']
        b_score += rnd['player_b'][b_strat]['player']['play']['my_score']

    if player_name[-1]==matchup_name[-1]:
        return b_score, a_score
    else:
        return a_score, b_score

def get_all_player_scores(players_struct):
    players = list(players_struct.keys())
    all_match_scores = {}
    for player_name in players:
        all_match_scores = get_player_score(players_struct,player_name,all_match_scores)
    all_match_scores    
    return all_match_scores

def get_player_score(players_struct, player_name, all_match_scores):
    matchups = list(players_struct[player_name].keys())
    all_match_scores[player_name] = 0

    for matchup_name in matchups:
        rounds = load_matchup_files(players_struct,player_name, matchup_name)
        all_match_scores[player_name] += get_matchup_scores(rounds,player_name,matchup_name)[0]
    return all_match_scores

def get_matchup_strats(matchup,player_name,matchup_name):
    a_strat = list(matchup[0]['player_a'].keys())[0]
    b_strat = list(matchup[0]['player_b'].keys())[0]
    #need to check that player name is in the right order
    if player_name[-1]==matchup_name[-1]:
        return b_strat, a_strat
    else:
        return a_strat, b_strat
