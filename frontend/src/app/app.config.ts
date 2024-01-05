import { ApplicationConfig } from '@angular/core';
import { provideRouter } from '@angular/router';

import { routes } from './app.routes';
import { GAME_PROVIDER } from 'src/domain/game.provider.interface';
import { GameProviderService } from 'src/providers/game.provider.service';

export const appConfig: ApplicationConfig = {
  providers: [
    provideRouter(routes),
    {
      provide: GAME_PROVIDER,
      useClass: GameProviderService
    }
    
  
  ]
};
