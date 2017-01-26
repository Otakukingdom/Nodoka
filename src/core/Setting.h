//
// Created by mistlight on 1/8/17.
//

#ifndef NODOKANATIVE_SETTING_H
#define NODOKANATIVE_SETTING_H

#include <QSettings>

namespace Core {
    class Setting : public QObject {
    Q_OBJECT

        QSettings* setting;

    public:
        Setting();


        int getVolume();
        int getCurrentAudiobookId();
        QString getSpeed();
        void setSpeed(QString speed);

    signals:
        void volumeUpdated(int volume);

    public slots:
        void setVolume(int volume);
        void setCurrentAudiobook(int audiobookId);
    };
}



#endif //NODOKANATIVE_SETTING_H
