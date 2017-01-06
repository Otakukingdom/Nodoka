//
// Created by mistlight on 1/2/2017.
//

#ifndef NODOKANATIVE_AUDIOBOOKFILERECORD_H
#define NODOKANATIVE_AUDIOBOOKFILERECORD_H

#include <QtSql>

class AudiobookFileRecord : public QSqlRecord {

private:
    bool readMode;

    void setup();

public:
    AudiobookFileRecord(bool readMode);

    void setInitValues();
};


#endif //NODOKANATIVE_AUDIOBOOKFILERECORD_H
