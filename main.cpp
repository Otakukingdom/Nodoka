#include <QApplication>
#include <QtWidgets/QPushButton>
#include <zconf.h>
#include <assert.h>
#include "vlc/vlc.h"

int main(int argc, char *argv[]) {
    libvlc_instance_t * inst;
    libvlc_media_player_t *mp;
    libvlc_media_t *m;

     /* Load the VLC engine */
    inst = libvlc_new (0, NULL);

     /* Create a new item */
     // m = libvlc_media_new_location (inst, "http://mycool.movie.com/test.mov");
    m = libvlc_media_new_path (inst, "E:\\test.mp3");

    assert(m != NULL);

     /* Create a media player playing environement */
    mp = libvlc_media_player_new_from_media (m);

     /* No need to keep the media now */
    libvlc_media_release (m);

 #if 0
     /* This is a non working code that show how to hooks into a window,
      * if we have a window around */
      libvlc_media_player_set_xwindow (mp, xid);
     /* or on windows */
      libvlc_media_player_set_hwnd (mp, hwnd);
     /* or on mac os */
      libvlc_media_player_set_nsobject (mp, view);
  #endif

     /* play the media_player */
    libvlc_media_player_play (mp);

    QApplication a(argc, argv);

    QPushButton button ("Hello world !");
    button.show();

    return a.exec();
}