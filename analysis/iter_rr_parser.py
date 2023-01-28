import sys,os 
import matplotlib.pyplot as plt 


# this is very hardcoded and bad and not meant to go anywhere
start = [200,200,200,200,200]
rnd = [1,2,3,4,5]
strat_labels =  ["AlwaysDefect", "GrimTrigger", "TitForTat","RandomDefect", "TitForAverageTat"]
line_labels  =  ["-", ".-", "--",".--", ".-."]
chance = -1;
data = []
data.append(start)
with open(sys.argv[1], 'r') as f:
        for line in f: 
                if len(line) < 5: 
                        # is start of new dataset
                        if len(data) < 2:
                                chance = int(line)
                                start = [200,200,200,200,200]
                                continue
                        parse = [list(),list(),list(),list(),list()] 
                        for r in range(len(data)): 
                            for i in range(len(data[r])):
                                parse[i].append(data[r][i]) 
                                
                        c = 0 
                        print(parse)
                        for strat in parse:
                                colour = "C{}".format(c); 
                                plt.plot(rnd, strat, line_labels[c]  , linewidth=1, label=strat_labels[c], color=colour); 
                                c = c + 1
                        plt.legend()
                        plt.title(chance)
                        #plt.show()
                        name = "{}".format(int(chance))
                        
                        plt.savefig(name)
                        plt.close()
                        print(chance,data)
                        print("=========================")
                        chance = int(line);
                        data = []
                        data.append(start) 
                else:
                        # more data
                        raw = line.split(" ")
                        for i in range(len(raw)):
                             raw[i] = int(raw[i])
                        data.append(raw)
                
