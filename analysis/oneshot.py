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
#basebasedir = "/tmp/test_runs"
#basebasedir = "/home/henry/runs/matchups_tfat_alldefect"
#basebasedir = "/home/henry/runs/matchups_tfat_grimtrigger"
#basebasedir = "/home/henry/runs/matchups_tfat_tft"
#basebasedir = "/home/henry/runs/matchups_tfat_randomdefect"
#basebasedir = "/home/henry/runs/matchups_tfat_tfat"

basebasedir = "/home/henry/runs/testrun"


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
                #list every file 
                print(run)
                for match in listdir(basebasedir + "/" + run):
                    for rnd in listdir(basebasedir + "/" + run + "/" + match):
                        path = basebasedir + "/" + run + "/" + match + "/" + rnd
                        f = open(path)
                        jason = json.load(f)
                        f.close()
                        a_strat = list(jason['player_a'].keys())[0]
                        b_strat = list(jason['player_b'].keys())[0]
                        a_score = jason['player_a'][a_strat]['player']['play']['my_score']
                        b_score = jason['player_b'][b_strat]['player']['play']['my_score']
                        strat_sums[a_strat]['sum'] += a_score
                        strat_sums[b_strat]['sum'] += b_score
                        strat_sums[a_strat]["sumVs" + b_strat] += a_score
                        strat_sums[b_strat]["sumVs" + a_strat] += b_score
                        strat_sums[a_strat]['chance'] = jason['noisemodel']['chance']
                        strat_sums[b_strat]['chance'] = jason['noisemodel']['chance']
                        # print(match + " " + rnd)
                print(strat_sums) 
                final.append(strat_sums)
        return final

def mysort(e):
        return e[1]

data = oneshot()
print(data) 


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

print(TitForTat,GrimTrigger,AlwaysDefect,RandomDefect,TitForAverageTat)
if len(TitForTat) > 0:
	y, x = zip(*TitForTat)
	plt.plot(x,y , 'o--y', linewidth=2, label='TitForTat', color='C0')
if len(GrimTrigger) > 0:
	y, x = zip(*GrimTrigger)
	plt.plot(x,y , 'o--y', linewidth=2, label='GrimTrigger', color='C1')
if len(AlwaysDefect) > 0:
	y, x = zip(*AlwaysDefect)
	plt.plot(x,y , 'o--y', linewidth=2, label='AlwaysDefect', color='C2')
if len(RandomDefect) > 0:
	y, x = zip(*RandomDefect)
	plt.plot(x,y , 'o--y', linewidth=2, label='RandomDefect', color='C3')
if len(TitForAverageTat) > 0:
	y, x = zip(*TitForAverageTat)
	plt.plot(x,y , 'o--y', linewidth=2, label='TitForAverageTat', color='C4')

plt.gca().invert_xaxis()
plt.legend()

plt.show()


