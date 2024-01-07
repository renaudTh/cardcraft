import { ApplicationConfig } from '@angular/core';
import { provideRouter } from '@angular/router';

import { routes } from './app.routes';
import { GAME_PROVIDER } from '../domain/game.provider.interface';
import { GameProviderService } from '../providers/game.provider.service';
import { CARD_PROVIDER } from 'src/domain/card.provider.interface';
import { CardProviderService } from 'src/providers/card.provider.service';


export const appConfig: ApplicationConfig = {
  providers: [
    provideRouter(routes),
    {
      provide: GAME_PROVIDER,
      useClass: GameProviderService
    },
    {
      provide: CARD_PROVIDER,
      useClass: CardProviderService,
    }
    
  
  ]
};
