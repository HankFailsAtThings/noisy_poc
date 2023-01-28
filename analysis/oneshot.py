#!/usr/bin/env python
import json 
from os import listdir
from noisygamesutil import *
import matplotlib.pyplot as plt

#basebasedir = '/home/henry/runs/run2_10players' # small dataset for testing this script
#basebasedir = '/home/henry/runs/run1_1000players' 
#basebasedir  = '/home/henry/runs/run3_100player'
#basebasedir  = '/home/henry/runs/run4_100player'
#basebasedir  = '/home/henry/runs/run5_100player'
#basebasedir  = '/home/henry/runs/titforAvgTat/run1_100player'
#basebasedir  = '/home/henry/runs/titforAvgTat/run2_100player'
basebasedir = "/tmp/test_runs"

#strat names 

def oneshot():
        final = []
        for run in listdir(basebasedir): 
                strat_sums = { 
                        'TitForTat'             : {'sum' : 0, 'sumVsTitForAverageTat' : 0, 'sumVsGrimTrigger' : 0, 'sumVsAlwaysDefect' : 0, 'sumVsRandomDefect' : 0 , 'sumVsTitForTat' : 0, 'chance' : 0}, 
                        'TitForAverageTat'      : {'sum' : 0, 'sumVsTitForAverageTat' : 0, 'sumVsGrimTrigger' : 0, 'sumVsAlwaysDefect' : 0, 'sumVsRandomDefect' : 0 , 'sumVsTitForTat' : 0, 'chance' : 0}, 
                        'GrimTrigger'           : {'sum' : 0, 'sumVsTitForAverageTat' : 0, 'sumVsGrimTrigger' : 0, 'sumVsAlwaysDefect' : 0, 'sumVsRandomDefect' : 0 , 'sumVsTitForTat' : 0, 'chance' : 0}, 
                        'AlwaysDefect'          : {'sum' : 0, 'sumVsTitForAverageTat' : 0, 'sumVsGrimTrigger' : 0, 'sumVsAlwaysDefect' : 0, 'sumVsRandomDefect' : 0 , 'sumVsTitForTat' : 0, 'chance' : 0},
                        'RandomDefect'          : {'sum' : 0, 'sumVsTitForAverageTat' : 0, 'sumVsGrimTrigger' : 0, 'sumVsAlwaysDefect' : 0, 'sumVsRandomDefect' : 0 , 'sumVsTitForTat' : 0, 'chance' : 0} }

                player_struct = build_players_struct(basebasedir + "/" +  run + "/")
                players = list(player_struct.keys())
                # for each player collect their score by matchup 
                for player in players:
                        matchups = player_struct[player].keys()
                        for matchup in matchups:
                                rounds = load_matchup_files(player_struct,player,matchup)
                                for rnd in rounds:
                                        a_strat = list(rnd['player_a'].keys())[0]
                                        b_strat = list(rnd['player_b'].keys())[0]
                                        a_score = rnd['player_a'][a_strat]['player']['play']['my_score']
                                        b_score = rnd['player_b'][b_strat]['player']['play']['my_score']
                                        strat_sums[a_strat]['sum'] += a_score
                                        strat_sums[b_strat]['sum'] += b_score
                                        strat_sums[a_strat]["sumVs" + b_strat] += a_score
                                        strat_sums[b_strat]["sumVs" + b_strat] += b_score
                                        strat_sums[a_strat]['chance'] = rnd['noisemodel']['chance'] # this is prob unsafe
                                        strat_sums[b_strat]['chance'] = rnd['noisemodel']['chance']
                
                final.append(strat_sums)              
                                        
        return final    
                                        
                                        
# boged, sort by chance        
def mysort(e):
        return e[1]

data = oneshot()

TitForTat    = []  
TitForAverageTat    = []  
GrimTrigger  = []
AlwaysDefect = []
RandomDefect = [] 
for d in data:
        print(d)
        TitForTat.append(list( [d['TitForTat']['sum'], d['TitForTat']['chance']]))
        TitForAverageTat.append(list( [d['TitForAverageTat']['sum'], d['TitForAverageTat']['chance']]))
        GrimTrigger.append(list([d['GrimTrigger']['sum'], d['GrimTrigger']['chance']]))
        AlwaysDefect.append(list([d['AlwaysDefect']['sum'], d['AlwaysDefect']['chance']]))
        RandomDefect.append(list([d['RandomDefect']['sum'], d['RandomDefect']['chance']]))


TitForTat.sort(key=mysort)
GrimTrigger.sort(key=mysort)
AlwaysDefect.sort(key=mysort)
RandomDefect.sort(key=mysort)
TitForAverageTat.sort(key=mysort)

#print(TitForTat,GrimTrigger,AlwaysDefect,RandomDefect,TitForAverageTat)
y, x = zip(*TitForTat)
print(TitForTat, y, x, type(y), type(x))
plt.plot(x,y , 'o--y', linewidth=2, label='TitForTat', color='C0')
y, x = zip(*GrimTrigger)
plt.plot(x,y , 'o--y', linewidth=2, label='GrimTrigger', color='C1')
y, x = zip(*AlwaysDefect)
plt.plot(x,y , 'o--y', linewidth=2, label='AlwaysDefect', color='C2')
y, x = zip(*RandomDefect)
plt.plot(x,y , 'o--y', linewidth=2, label='RandomDefect', color='C3')
y, x = zip(*TitForAverageTat)
plt.plot(x,y , 'o--y', linewidth=2, label='TitForAverageTat', color='C4')

plt.gca().invert_xaxis()
plt.legend()

plt.show()


