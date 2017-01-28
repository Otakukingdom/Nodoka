//
// Created by mistlight on 1/27/2017.
//

#ifndef NODOKANATIVE_AUDIOBOOKPROXY_H
#define NODOKANATIVE_AUDIOBOOKPROXY_H

#include <QSqlRecord>


class AudiobookProxy {
    QSqlRecord record;

public:
    AudiobookProxy(QSqlRecord record);
    void remove();
    void rescan();
};


#endif //NODOKANATIVE_AUDIOBOOKPROXY_H
