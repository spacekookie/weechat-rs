all:
	gcc -fPIC -Wall -c myplug.c
	gcc -shared -fPIC -o libplug.so myplug.o
	cp libplug.so ~/.weechat/plugins/plug
	
clean:
	rm -f myplug.o libplug.so
