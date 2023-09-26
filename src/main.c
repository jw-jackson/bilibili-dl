/***************************************************************************
*
*     Project                 ____  ____  ____  _     
*                            | __ )| __ )|  _ \| |    
*                            |  _ \|  _ \| | | | |    
*                            | |_) | |_) | |_| | |___ 
*                            |____/|____/|____/|_____|
*
*
*
***************************************************************************/
#define PROGNAME "BilibiliDownload"
#define VERSION "v0.0.0"


#include <stdio.h>
#include <string.h>
#include "login.h"
#include <stdlib.h>
#include "options.h"
#include "cJSON.h"
#include "download.h"

#ifdef DEBUG
#define DPRINTF(format,...) \
do {printf("%s::%s "format, __FILE__,__FUNCTION__,##__VA_ARGS__);} while(0)
#else
#define DPRINTF
#endif

#define STREQ(x, y) (strcmp(x, y) == 0)



// default
enum Cmd cmd = UNUSED;
struct Options options = {
    .version = false,
    .help = false,
    .loginMethod = QRCODE,
}; 
// 保存 url 数组
cJSON *urlArr = NULL;


static int ParasCmdOpt(int argc, char *argv[]);
static void PrintVersion();
static void PrintHelp();
int main(int argc, char* argv[])
{
    urlArr = cJSON_CreateArray();
    if (urlArr == NULL){
        return -1;
    }

    if (argc == 1){
        options.help = true;
    }
    else{
        if(ParasCmdOpt(argc, argv) != 0){
            exit(1);
        };
    }

    while(1){
        switch (cmd) {
            case UNUSED:
                // if opt not --version --help  continue ->  Download
                if(options.version){
                    //DPRINTF("cmd: Unused opt: version\n");
                    PrintVersion();

                }    
                else if(options.help){
                    PrintHelp();
                    //DPRINTF("cmd: Unused opt: help\n");
                }
                else {
                    cmd = DOWNLOAD;
                    continue;
                }
                break;
            case LOGIN:
                Login();
                break;
            case DOWNLOAD:
                printf("此功能还没有实现，可以在 issue 中催催作者.\n");
                Download();
                break;
            default:
                DPRINTF("cmd: error\n");
                exit(1);
        } 
        break;
    }
    cJSON_Delete(urlArr);
    return 0;
}

static int ParasCmdOpt(int argc, char *argv[]){
    
    // argv[0] is the program name
    argc--;
    argv++;
     
    while(argc--){
        // opt
        if( argv[0][0] == '-' ){
            // option
            // double --
            if(STREQ(argv[0], "-v") || STREQ(argv[0], "--version")){
                options.version = true;
            }
            else if(STREQ(argv[0], "-h") || STREQ(argv[0], "--help")){
                options.help = true;

            }

            else if(STREQ(argv[0], "--qrcode")){
                options.loginMethod = QRCODE;
            }
            else if(STREQ(argv[0], "--smscode")){
                options.loginMethod = SMSCODE;
            }
            else if(STREQ(argv[0], "--password")){
                options.loginMethod = PASSWORD;
            }


            else{
                // printf error
                printf("Unknow option: %s\n", argv[0]);
                printf(
                    "Try 'bbdl -h' for more informantion.\n"
                );
                return -1;
            }
            
        }

        // cmd
        else if(STREQ(argv[0], "login") || STREQ(argv[0], "Login")){
            cmd = LOGIN;
        }

        else if(STREQ(argv[0], "download") || STREQ(argv[0], "Download")){
            cmd = DOWNLOAD;
        }

        // url
        else{
            cJSON *url = cJSON_CreateString(argv[0]);
            cJSON_AddItemToArray(urlArr, url);
        }

        argv++;
    }

    return 0;
}


static void PrintHelp(){
    printf(
        "Usage: " "bbdl" " [command] [options] <URL>\n"
        "\n"
        "options:\n"
        "  -v, --version        print program version\n"
        "  -h, --help           give this help list\n"
        "  --qrcode             login by QR Code\n"
        "  --smscode            login by SMS Code\n"
        "  --password           login by password\n"
        "\n"
        "commands:\n"
        "  login                login in account\n"
        "  download             download,default\n"
    );
}

static void PrintVersion(){
    printf(
        PROGNAME " " VERSION "\n"
        "See more informantion https://github.com/jw-jackson/bilibili-dl.\n"
    );
}

