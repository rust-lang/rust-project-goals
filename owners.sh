ls src/2024h2/*.md | while read f; do
    if [ "$f" == "src/2024h2/README.md" ]; then
        continue
    fi
    if [ "$f" == "src/2024h2/accepted.md" ]; then
        continue
    fi
    if [ "$f" == "src/2024h2/not_accepted.md" ]; then
        continue
    fi
    if [ "$f" == "src/2024h2/notes.md" ]; then
        continue
    fi
    if [ "$f" == "src/2024h2/candidates.md" ]; then
        continue
    fi
    if [ "$f" == "src/2024h2/flagship.md" ]; then
        continue
    fi
	grep -H "Owner(s)" $f | cut -d"|" -f1,3 | grep -v "or team" | grep -v Help | grep -v Github | sed 's#src/2024h2/##g' 
done | gawk '
BEGIN { n=1;
   teams = ""
   goal = ""
   file = "teams.txt"
   while ((getline line < file) != EOF) {
	   if (!match (line, /^ /)) {
	   		if (goal != "")
			   goal_teams[goal]=teams
			goal = line
			teams=""
	   } else {
	   		m = split(line, gteams, /,/)
			for (i=1; i<=m; i++) {
				tm = tolower(gteams[i])
				gsub(/\[/, "", tm)
				gsub(/\]/, "", tm)
				gsub(/ /, "", tm)
		   		teams=teams tm " "
				team_goals[tm] = team_goals[tm] " " goal
			}
	   }
   }
}
{
	split($1, a, /:/);
	goal=a[1];
	gsub(/.md/, "", goal)
	owner = $2
	gsub(/,/, "", owner)
	actor = "agent {" owner "}" " { }\n";
	actors[actor] = "";
	task = "task {" goal "}\n"
	if (tasks[actor]) {
		tasks[actor] = tasks[actor] task;
	} else {
		tasks[actor] = task;
	}
}
END {
	asorti(actors, s_actors)
	l = length(s_actors)
	for (ai=0; ai<l; ai++) {
		actor = s_actors[ai]
		if (actor == "")
			continue
		text = ""
		goal = "goal {Rust} &\n"
		if (depends) {
			depends = depends "," "~>" n
		} else {
			depends = "~>" n
		}
		text = (n++) " " actor 
		lines[n] = text
		actor_id = n
		text = " " (n++) " " goal
		lines[n] = text
		m = split(tasks[actor], b, "\n");
		for (i=1; i<m; i++) {
			split(b[i], c, "{")
			task = c[2]
			gsub(/\}/, "", task)
			task_ids[task] = n
			actor_ids[task] = actor_id
			text = "    " (n++) " " b[i] "\n"
			lines[n] = text
		}
	}
	for (t in team_goals) {
		m = split(team_goals[t], gs, " ") 
		team_actor = (n++) " role {" t "}" 
		i_actor = n - 1
		lines[n] = team_actor
		for (i=1; i<=m; i++) {
			text="  " (n++) " goal {" gs[i] "}" 
			lines[n] = text
			dependums[gs[i]] = dependums[gs[i]] " " n
		}
	}
	for (dep in dependums) {
		text = (n++) " task {" dep "}" 
		x = split(dependums[dep], dependers, " ");
		for (ix = 1; ix <= x; ix++) {
		 depender = dependers[ix]
		 if (depender) {
		   lines[depender] = substr(lines[depender], 0, length(lines[depender])) " ~>" (n-1) "\n"
		 }
 	        }
		x = split(actor_ids[dep], dependees, " ");
		for (ix = 1; ix <= x; ix++) {
		   dependee = dependees[ix] - 1
		   if (dependee) {
			text = text " ~>" dependee
		   }
		}
		lines[n] = text "\n"
	}
	for (i=0; i<=n; i++) {
		print lines[i]
	}
}
' | sed '/^$/d' > pistar/istar2/goalModel.istar2
# cat huawei.istar2 >> pistar/istar2/goalModel.istar2
cp goalModel.istar2 pistar/istar2/goalModel.istar2
cd pistar/istar2
make
cd -

