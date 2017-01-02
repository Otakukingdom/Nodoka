//
// Created by mistlight on 1/2/2017.
//

#ifndef NODOKANATIVE_AUDIOBOOKFILE_H
#define NODOKANATIVE_AUDIOBOOKFILE_H

#include <QSqlTableModel>

class AudiobookFile : public QSqlTableModel {

    AudiobookFile(QObject *parent = 0);
    void addAudiobookFile(QString path);

};


#endif //NODOKANATIVE_AUDIOBOOKFILE_H
