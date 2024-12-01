# Copied from https://gist.github.com/klmr/575726c7e05d8780505a?permalink_comment_id=4562279#gistcomment-4562279
.PHONY: help
help:           ## Show this help.
	@echo "$$(tput setaf 2)Available rules:$$(tput sgr0)";sed -ne"/^## /{h;s/.*//;:d" -e"H;n;s/^## /---/;td" -e"s/:.*//;G;s/\\n## /===/;s/\\n//g;p;}" ${MAKEFILE_LIST}|awk -F === -v n=$$(tput cols) -v i=4 -v a="$$(tput setaf 6)" -v z="$$(tput sgr0)" '{printf"- %s%s%s\n",a,$$1,z;m=split($$2,w,"---");l=n-i;for(j=1;j<=m;j++){l-=length(w[j])+1;if(l<= 0){l=n-i-length(w[j])-1;}printf"%*s%s\n",-i," ",w[j];}}'

.PHONY: dev-reload
## Run HTTP server with auto-reload features. Requires systemfd and cargo-watch
dev-reload: 
	systemfd --no-pid -s http::7112 -- cargo watch -x run 

.PHONY: tailwind
## Run tailwindcss --watch
tailwind: 
	tailwindcss -i input.css -o assets/output.css --watch
